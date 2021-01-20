  within VCRCycle;
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
        final quantity="Quality",
        final unit="mol/mol",
        displayUnit="mol/mol",
        max=1,
        min=-1,
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