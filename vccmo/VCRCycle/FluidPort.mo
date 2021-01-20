within VCRCycle;
  model FluidPort "FluidPort"

   connector FluidPortPH
      Units.Pressure p; 
      Units.SpecificEnthalpy h;
      Units.MassFlowRate mdot;
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
