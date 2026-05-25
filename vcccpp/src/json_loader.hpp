/*
  JSON Loader Helper - Manages string lifecycle safely
*/

#ifndef JSON_LOADER_HPP
#define JSON_LOADER_HPP

#include "common.hpp"
#include "vcc.hpp"
#include <fstream>
#include <sstream>
#include <regex>

using namespace std;

class JSONLoader {
private:
    vector<string> stringPool;
    string jsonContent;
    
    // 辅助函数：提取字符串值
    string extractString(size_t& pos) {
        pos++; // 跳过开始引号
        size_t start = pos;
        while (pos < jsonContent.length() && jsonContent[pos] != '"') {
            pos++;
        }
        string result = jsonContent.substr(start, pos - start);
        pos++; // 跳过结束引号
        return result;
    }
    
    // 辅助函数：提取数值
    double extractNumber(size_t& pos) {
        size_t start = pos;
        while (pos < jsonContent.length() && (isdigit(jsonContent[pos]) || jsonContent[pos] == '.' || jsonContent[pos] == '-')) {
            pos++;
        }
        return stod(jsonContent.substr(start, pos - start));
    }
    
    // 解析端口
    mPort parsePort(size_t& pos) {
        mPort port;
        
        // 找到 {
        while (pos < jsonContent.length() && jsonContent[pos] != '{') pos++;
        pos++;
        
        while (pos < jsonContent.length()) {
            // 跳过空白
            while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r' || jsonContent[pos] == ',')) pos++;
            
            if (pos >= jsonContent.length() || jsonContent[pos] == '}') {
                pos++;
                break;
            }
            
            // 提取键
            string key = extractString(pos);
            
            // 找到 :
            while (pos < jsonContent.length() && jsonContent[pos] != ':') pos++;
            pos++;
            
            // 跳过空白
            while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r')) pos++;
            
            // 提取值
            if (isdigit(jsonContent[pos]) || jsonContent[pos] == '-') {
                double value = extractNumber(pos);
                port[key] = value;
            }
        }
        
        return port;
    }
    
    // 解析组件
    umComponent parseComponent(size_t& pos) {
        umComponent comp;
        
        // 找到 {
        while (pos < jsonContent.length() && jsonContent[pos] != '{') pos++;
        pos++;
        
        while (pos < jsonContent.length()) {
            // 跳过空白
            while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r' || jsonContent[pos] == ',')) pos++;
            
            if (pos >= jsonContent.length() || jsonContent[pos] == '}') {
                pos++;
                break;
            }
            
            // 提取键
            string key = extractString(pos);
            
            // 找到 :
            while (pos < jsonContent.length() && jsonContent[pos] != ':') pos++;
            pos++;
            
            // 跳过空白
            while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r')) pos++;
            
            // 提取值
            if (jsonContent[pos] == '{') {
                // 对象（端口）
                mPort port = parsePort(pos);
                comp[key] = port;
            } else if (jsonContent[pos] == '"') {
                // 字符串 - 存储为 string 对象
                string value = extractString(pos);
                stringPool.push_back(value); // 保持引用
                comp[key] = stringPool.back(); // 存储 string 而不是 const char*
            } else if (isdigit(jsonContent[pos]) || jsonContent[pos] == '-') {
                // 数值
                double value = extractNumber(pos);
                comp[key] = value;
            }
        }
        
        return comp;
    }

public:
    // 从文件读取
    bool loadFile(const string& filename) {
        ifstream file(filename);
        if (!file.is_open()) {
            cerr << "Could not open file: " << filename << endl;
            return false;
        }
        stringstream buffer;
        buffer << file.rdbuf();
        jsonContent = buffer.str();
        return true;
    }
    
    // 解析JSON并创建VCCycle
    unique_ptr<VCCycle> createCycle() {
        vector<umComponent> components;
        vector<tupConnector> connectors;
        size_t pos = 0;
        
        // 解析 components
        regex componentsRegex("\"components\"\\s*:\\s*\\[");
        smatch match;
        if (regex_search(jsonContent, match, componentsRegex)) {
            pos = match.position() + match.length();
            
            while (pos < jsonContent.length()) {
                // 跳过空白
                while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r' || jsonContent[pos] == ',')) pos++;
                
                if (pos >= jsonContent.length() || jsonContent[pos] == ']') {
                    pos++;
                    break;
                }
                
                // 提取组件对象
                umComponent comp = parseComponent(pos);
                components.push_back(comp);
            }
        }
        
        // 解析 connectors
        regex connectorsRegex("\"connectors\"\\s*:\\s*\\{");
        if (regex_search(jsonContent, match, connectorsRegex)) {
            pos = match.position() + match.length();
            
            while (pos < jsonContent.length()) {
                // 跳过空白
                while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r' || jsonContent[pos] == ',')) pos++;
                
                if (pos >= jsonContent.length() || jsonContent[pos] == '}') break;
                
                // 提取键（例如 "Compressor.oPort"）
                string key = extractString(pos);
                
                // 分割键为组件名和端口名
                size_t dotPos = key.find('.');
                string comp1 = key.substr(0, dotPos);
                string port1 = key.substr(dotPos + 1);
                stringPool.push_back(comp1);
                stringPool.push_back(port1);
                
                // 找到 :
                while (pos < jsonContent.length() && jsonContent[pos] != ':') pos++;
                pos++;
                
                // 跳过空白
                while (pos < jsonContent.length() && (jsonContent[pos] == ' ' || jsonContent[pos] == '\t' || jsonContent[pos] == '\n' || jsonContent[pos] == '\r')) pos++;
                
                // 提取值（例如 "Condenser.iPort"）
                string value = extractString(pos);
                dotPos = value.find('.');
                string comp2 = value.substr(0, dotPos);
                string port2 = value.substr(dotPos + 1);
                stringPool.push_back(comp2);
                stringPool.push_back(port2);
                
                // 添加连接器 - 使用池中的字符串
                connectors.push_back(make_tuple(
                    make_tuple(stringPool[stringPool.size()-4], stringPool[stringPool.size()-3]),
                    make_tuple(stringPool[stringPool.size()-2], stringPool[stringPool.size()-1])
                ));
            }
        }
        
        return make_unique<VCCycle>(components, connectors);
    }
    
    // 获取字符串池（用于调试）
    const vector<string>& getStringPool() const { return stringPool; }
};

#endif // JSON_LOADER_HPP
