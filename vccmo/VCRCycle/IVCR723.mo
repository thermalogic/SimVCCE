within VCRCycle;
model IVCR723 "IVCR Exampl23"
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
     annotation (Placement(transformation(extent={{-28,-27},{28,27}}, origin={48,
            11})));
   VCRCycle.Components.Condenser_ph condenser_ph(tout=t3,xout=x3)
     annotation (Placement(visible = true, transformation(extent={{-22,-26},{22,
            26}},                                                                         rotation = 0,
        origin={-6,44})));
   Components.ExpansionValve_ph expansionValve_ph
     annotation (Placement(transformation(extent={{-10,-10},{10,10}}, origin={-70,
            12})));
   VCRCycle.Components.Evaporator_ph evaporator_ph
     annotation (Placement(visible = true, transformation(origin={-2,-53},    extent={{-22,-21},
            {22,21}},                                                                                          rotation = 0)));
equation
   Wc=compressor_ph.wc;
   Qlow=evaporator_ph.Qlow*60/211;
   cop = evaporator_ph.Qlow / Wc;
 connect(condenser_ph.inlet, compressor_ph.outlet) annotation (
     Line(points={{13.58,54.66},{28,54.66},{28,24.5},{31.2,24.5}},
                                                        color = {0, 131, 169}, thickness = 0.5));
 connect(condenser_ph.outlet, expansionValve_ph.inlet) annotation (
     Line(points={{-25.8,54.4},{-71,54.4},{-71,19}},  color = {0, 131, 169}, thickness = 0.5));
 connect(expansionValve_ph.outlet, evaporator_ph.inlet) annotation (
     Line(points={{-70.8,5},{-70.8,-57.2},{-21.8,-57.2}},  color = {0, 131, 169}, thickness = 0.5));
 connect(compressor_ph.inlet, evaporator_ph.outlet) annotation (
     Line(points={{64.8,-7.9},{64.8,-57.62},{17.8,-57.62}}, color = {0, 131, 169}, thickness = 0.5));
   annotation (Icon(coordinateSystem(preserveAspectRatio=false), graphics={
           Text(
           extent={{-82,52},{76,-48}},
           lineColor={28,108,200},
           textString="IVCR1",
           fontSize=24,
           textStyle={TextStyle.Bold}), Ellipse(extent={{-56,60},{50,-58}},
             lineColor={28,108,200})}),                           Diagram(
         coordinateSystem(preserveAspectRatio=false)),
    experiment(StopTime=0, __Dymola_Algorithm="Dassl"));
end IVCR723;
