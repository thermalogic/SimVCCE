within ;
package VCRCycle "Simulation of VCR Cycle"
extends Modelica.Icons.Package;
  model FluidPort "FluidPort"

   connector FluidPortPH
      Units.Pressure p "Pressure";
      Units.SpecificEnthalpy h "Specific enthalpy";
      Units.MassFlowRate mdot "Mass Fraction";
   end FluidPortPH;

    connector FluidPortInPH
      extends FluidPortPH;
      annotation (Icon(graphics={Ellipse(
              extent={{-98,96},{102,-104}},
              fillColor={0,131,169},
              fillPattern=FillPattern.Solid,
              pattern=LinePattern.Solid,
              lineColor={0,131,169},
              lineThickness=0.5)}));
    end FluidPortInPH;

    connector FluidPortOutPH
      extends FluidPortPH;
      annotation (Icon(graphics={Ellipse(
              extent={{-98,98},{98,-100}},
              fillColor={255,255,255},
              fillPattern=FillPattern.Solid,
              pattern=LinePattern.Solid,
              lineColor={0,131,169},
              lineThickness=0.5)}));
    end FluidPortOutPH;
    annotation (Icon(coordinateSystem(preserveAspectRatio=true, extent={{-100,-100},
              {100,100}}), graphics={
          Polygon(fillColor={102,102,102},
            pattern=LinePattern.None,
            fillPattern=FillPattern.Solid,
            points={{-98,24},{-58,24},{-28,74},{-8,74},{-8,-66},{-28,-66},{-58,
                -16},{-98,-16}}),
          Polygon(origin={16,6},
            lineColor={64,64,64},
            fillColor={255,255,255},
            fillPattern=FillPattern.Solid,
            points={{-10.0,70.0},{10.0,70.0},{40.0,20.0},{80.0,20.0},{80.0,-20.0},{40.0,-20.0},{10.0,-70.0},{-10.0,-70.0}})}),
                                                                 Diagram(
          coordinateSystem(preserveAspectRatio=true, extent={{-100,-100},{100,100}}),
          graphics));
  end FluidPort;

  model Units "Types with custom units"
    type Pressure = Real (
        final quantity="Pressure",
        final unit="MPa",
        displayUnit="MPa",
        nominal=1e5,
        min=0);

    type Temperature_DegC = Real (
        final quantity="Temperature",
        final unit="degC",
        displayUnit="degC",
        min=-273.15,
        start=0.0);

    type SpecificEnthalpy = Real (
        final quantity="SpecificEnthalpy",
        final unit="kJ/kg",
        displayUnit="kJ/kg");

    type SpecificEntropy = Real (
        final quantity="SpecificEntropy",
        final unit="kJ/(kg.K)",
        displayUnit="kJ/(kg.K)");

    type MassFlowRate = Real (
        final quantity="MassFlowRate",
        final unit="kg/s",
        displayUnit="kg/s");

    type Power = Real (
        final quantity="Power",
        final unit="kW",
        displayUnit="kW",
        min=0);

    type Quality =      Real (
        final quantity="Dryness",
        final unit="",
        displayUnit="",
        max=1,
        min=0,
        start=-1);

    type RefrigerationCapacity =      Real (
        final quantity="Refrigeration Capacity",
        final unit="ton",
        displayUnit="ton",
        min=0);

    type CoefficientPerformance =      Real (
        final quantity="The coefficient of performance",
        final unit="",
        displayUnit="",
        min=0);

    annotation (Icon(graphics={
          Rectangle(
            lineColor={200,200,200},
            fillColor={248,248,248},
            fillPattern=FillPattern.HorizontalCylinder,
            extent={{-104,-100},{96,100}},
            radius=25.0),
        Polygon(
          fillColor = {128,128,128},
          pattern = LinePattern.None,
          fillPattern = FillPattern.Solid,
          points={{-70,-30},{-70,-30},{-45,60},{-42.5,72.5},{-55,70},{-55,75},{
                -25,87.5},{-22.5,70},{-40,10},{-40,10},{-20,25},{-10,37.5},{-22.5,
                37.5},{-22.5,37.5},{-22.5,42.5},{-22.5,42.5},{12.5,42.5},{12.5,
                42.5},{12.5,37.5},{12.5,37.5},{2.5,37.5},{-20,17.5},{-20,17.5},{
                -15,-15},{-7.5,-18.75},{0,-15},{5,-16.25},{5,-22.5},{-6.25,-31.25},
                {-21.25,-33.75},{-30,-23.75},{-35,5},{-35,5},{-42.5,0},{-42.5,0},
                {-50,-30},{-50,-30}},
          smooth = Smooth.Bezier),
        Polygon(
          fillColor = {128,128,128},
          pattern = LinePattern.None,
          fillPattern = FillPattern.Solid,
          points={{97.5,40},{72.5,40},{72.5,40},{65,43.75},{46.25,45},{26.25,35},
                {17.5,16.25},{21.25,2.5},{32.5,-2.5},{32.5,-2.5},{16.25,-12.5},{
                16.25,-25},{26.25,-28.75},{26.25,-28.75},{31.25,-31.25},{31.25,
                -31.25},{55,-38.75},{57.5,-51.25},{42.5,-60},{22.5,-55},{17.5,
                -41.25},{31.25,-31.25},{31.25,-31.25},{26.25,-28.75},{26.25,
                -28.75},{16.25,-31.25},{3.75,-40},{6.25,-58.75},{40,-66.25},{75,
                -52.5},{73.75,-25},{37.5,-16.25},{32.5,-10},{37.5,-5},{37.5,-5},{
                40,2.5},{40,2.5},{37.5,7.5},{38.75,21.25},{46.25,37.5},{57.5,40},
                {63.75,32.5},{61.25,18.75},{55,3.75},{45,-1.25},{40,2.5},{40,2.5},
                {37.5,-5},{37.5,-5},{53.75,-6.25},{75,3.75},{82.5,20},{80,30},{80,
                30},{90,30}},
          smooth = Smooth.Bezier)}));
  end Units;

  package Components "Compressor,Condenser,etc"
  extends Modelica.Icons.Package;

    model Compressor_ph

      import R134a = Modelica.Media.R134a.R134a_ph;
      parameter Units.MassFlowRate mdotin;
      parameter Units.Temperature_DegC tin;
      parameter Units.Quality xin;
      Units.Power wc "Compressor Work";

      FluidPort.FluidPortInPH inlet annotation (Placement(transformation(
              extent={{50,-80},{70,-60}}), iconTransformation(extent={{50,-80},{70,-60}})));
      FluidPort.FluidPortOutPH outlet annotation (Placement(transformation(
              extent={{-70,40},{-50,60}}), iconTransformation(extent={{-70,40},{-50,
                60}})));
    equation
      inlet.h=R134a.specificEnthalpy(R134a.setState_Tx(tin+273.15,xin))/1000;

      outlet.h=R134a.specificEnthalpy_ps(outlet.p*1.0e6,R134a.specificEntropy(R134a.setState_Tx(tin+273.15,xin)))/1000;
      inlet.mdot=mdotin;
      outlet.mdot=inlet.mdot;
// process
      wc = inlet.mdot*(outlet.h-inlet.h);

       annotation (Icon(coordinateSystem(preserveAspectRatio=false), graphics={
              Polygon(points={{60,80},{60,-60},{-60,-40},{-60,40},{60,80}},
                lineColor={28,108,200})}), Diagram(coordinateSystem(
              preserveAspectRatio=false), graphics={Polygon(points={{60,80},{60,-60},
                  {-60,-40},{-60,40},{60,80}}, lineColor={28,108,200})}));
    end Compressor_ph;

    model Condenser_ph
      import R134a = Modelica.Media.R134a.R134a_ph;
      parameter Units.Pressure tout;
      parameter Units.Quality xout;

      FluidPort.FluidPortOutPH outlet
        annotation (Placement(transformation(extent={{-100,30},{-80,50}}),
            iconTransformation(extent={{-100,30},{-80,50}})));
      VCRCycle.FluidPort.FluidPortInPH inlet
        annotation (Placement(visible = true,transformation(extent = {{80, 32}, {98, 50}}, rotation = 0),
            iconTransformation(extent = {{80, 30}, {98, 48}}, rotation = 0)));

    equation
      outlet.p=R134a.saturationPressure(tout+273.15)/1.0e6;
      outlet.h=R134a.specificEnthalpy(R134a.setState_Tx(tout+273.15, xout))/1000;
      outlet.mdot =inlet.mdot;
      inlet.p=outlet.p
      annotation (Icon(coordinateSystem(preserveAspectRatio=false), graphics={
              Rectangle(extent={{-80,60},{60,-18}}, lineColor={28,108,200})}),
          Diagram(coordinateSystem(preserveAspectRatio=false), graphics={Rectangle(
                extent={{-72,60},{72,-24}}, lineColor={28,108,200})}));

      annotation (Diagram(graphics={Rectangle(origin = {0.192973, 10.0107}, extent = {{-80.0803, 49.9893}, {80.0803, -49.9893}})}), Icon(
            graphics={Rectangle(origin = {-0.4, 9.68}, lineColor = {0, 85, 255}, extent = {{-79.6, 50.32}, {79.6, -50.32}})}));
    end Condenser_ph;

    model ExpansionValve_ph
      FluidPort.FluidPortInPH inlet
        annotation (Placement(transformation(extent={{-20,60},{0,80}}),
            iconTransformation(extent={{-20,60},{0,80}})));
      FluidPort.FluidPortOutPH outlet annotation (Placement(transformation(
              extent={{-18,-80},{2,-60}}),  iconTransformation(extent={{-18,-80},
                {2,-60}})));
    equation
      outlet.h=inlet.h;
      outlet.mdot=inlet.mdot;
      annotation (Icon(coordinateSystem(preserveAspectRatio=false), graphics={Line(
                points={{-60,60},{40,60},{-59.9024,-59.8848},{40,-60},{-60,60}},
                        color={28,108,200})}), Diagram(coordinateSystem(
              preserveAspectRatio=false), graphics={Line(points={{-62,60},{42,60},{-60,
                  -60},{42,-60},{-62,60}}, color={28,108,200})}));
    end ExpansionValve_ph;

    model Evaporator_ph
      Real Qlow;
      FluidPort.FluidPortInPH inlet
        annotation (Placement(transformation(extent={{-100,-30},{-80,-10}}),
            iconTransformation(extent={{-100,-30},{-80,-10}})));
      VCRCycle.FluidPort.FluidPortOutPH outlet
        annotation (Placement(visible = true,transformation(extent = {{60, -30}, {80, -10}}, rotation = 0),
            iconTransformation(extent = {{82, -32}, {102, -12}}, rotation = 0)));

    equation
       Qlow=inlet.mdot * ( outlet.h - inlet.h);
      connect(outlet, outlet) annotation (Line(
          points={{70,-20},{72,-20},{72,-18},{74,-18},{74,-20},{70,-20}},
          color={0,131,169},
          pattern=LinePattern.Solid,
          thickness=0.5));
      annotation (Icon(coordinateSystem(preserveAspectRatio=false), graphics={Rectangle(origin = {-0.24, 10.4508}, lineColor = {85, 85, 255}, extent = {{79.93, 50.1492}, {-79.93, -50.1492}})}),
          Diagram(coordinateSystem(preserveAspectRatio=false), graphics={Rectangle(
                extent={{-70,40},{50,-36}}, lineColor={28,108,200}),
            Line(points={{-70,-20},{-60,-20}}, color={28,108,200}),
            Ellipse(
              extent={{-40,-12},{-20,-28}},
              lineColor={28,108,200},
              startAngle=0,
              endAngle=180,
              closure=EllipseClosure.None),
            Line(points={{-40,20},{-40,-20}}, color={28,108,200}),
            Ellipse(
              extent={{0,-12},{20,-28}},
              lineColor={28,108,200},
              startAngle=0,
              endAngle=180,
              closure=EllipseClosure.None),
            Ellipse(
              extent={{-60,28},{-40,12}},
              lineColor={28,108,200},
              startAngle=180,
              endAngle=360,
              closure=EllipseClosure.None),
            Ellipse(
              extent={{-20,28},{2,12}},
              lineColor={28,108,200},
              startAngle=180,
              endAngle=360,
              closure=EllipseClosure.None),
            Ellipse(
              extent={{20,26},{40,10}},
              lineColor={28,108,200},
              startAngle=180,
              endAngle=360,
              closure=EllipseClosure.None),
            Line(points={{-20,20},{-20,-20}}, color={28,108,200}),
            Line(points={{0,22},{0,-20}}, color={28,108,200}),
            Line(points={{20,20},{20,-22}}, color={28,108,200}),
            Line(points={{40,20},{40,-20}}, color={28,108,200}),
            Line(points={{40,-20},{50,-20}}, color={28,108,200}),
            Line(points={{-60,20},{-60,-20}}, color={28,108,200})}));
    end Evaporator_ph;
  annotation (Icon(graphics={
      Polygon(
        origin={4,34},
        lineColor = {0,0,127},
        fillColor = {0,0,127},
        fillPattern = FillPattern.Solid,
        points = {{-16,10},{4,10},{-6,-10},{-16,10}}),
      Line(
        origin={4,34},
        points = {{-6,-10},{-6,-40},{-6,-38}},
        color = {0,0,127}),
      Polygon(
        origin={4,32},
        lineColor = {0,0,255},
        fillColor = {0,128,255},
        fillPattern = FillPattern.Solid,
        points = {{-56,10},{-56,-90},{-6,-40},{44,10},{44,-90},{-56,10}})}));
  end Components;

  model IVCR1
    import R134a = Modelica.Media.R134a.R134a_ph;
    parameter Units.MassFlowRate mdot=0.08;
    parameter Units.Temperature_DegC t1=0;
    parameter Units.Temperature_DegC t3=26;
    parameter Units.Quality x1=1;
    parameter Units.Quality x3=0;
    // Variables
    Units.Power Wc;
    Units.RefrigerationCapacity Qlow;
    Units.CoefficientPerformance cop;

    Components.Compressor_ph compressor_ph(mdotin=mdot,tin=t1,xin=x1)
      annotation (Placement(transformation(extent={{34,-28},{76,22}})));
    VCRCycle.Components.Condenser_ph condenser_ph(tout=t3,xout=x3)
      annotation (Placement(visible = true, transformation(extent = {{-28, 18}, {16, 70}}, rotation = 0)));
    Components.ExpansionValve_ph expansionValve_ph
      annotation (Placement(transformation(extent={{-78,2},{-58,22}})));
    VCRCycle.Components.Evaporator_ph evaporator_ph
      annotation (Placement(visible = true, transformation(origin = {-2, -53}, extent = {{-22, -21}, {22, 21}}, rotation = 0)));
  equation
    Wc=compressor_ph.wc;
    Qlow=evaporator_ph.Qlow*60/211;
    cop = evaporator_ph.Qlow / Wc;
  connect(condenser_ph.inlet, compressor_ph.outlet) annotation(
      Line(points = {{14, 54}, {42.4, 54}, {42.4, 9.5}}, color = {0, 131, 169}, thickness = 0.5));
  connect(condenser_ph.outlet, expansionValve_ph.inlet) annotation(
      Line(points = {{-26, 54}, {-69, 54}, {-69, 19}}, color = {0, 131, 169}, thickness = 0.5));
  connect(expansionValve_ph.outlet, evaporator_ph.inlet) annotation(
      Line(points = {{-68.8, 5}, {-68.8, -57}, {-22, -57}}, color = {0, 131, 169}, thickness = 0.5));
  connect(compressor_ph.inlet, evaporator_ph.outlet) annotation(
      Line(points = {{67.6, -20.5}, {67.6, -58}, {18, -58}}, color = {0, 131, 169}, thickness = 0.5));
    annotation (Icon(coordinateSystem(preserveAspectRatio=false), graphics={
            Text(
            extent={{-82,52},{76,-48}},
            lineColor={28,108,200},
            textString="IVCR1",
            fontSize=24,
            textStyle={TextStyle.Bold}), Ellipse(extent={{-56,60},{50,-58}},
              lineColor={28,108,200})}),                           Diagram(
          coordinateSystem(preserveAspectRatio=false)));
  end IVCR1;
  annotation (uses(Modelica(version="3.2.3")));
end VCRCycle;
