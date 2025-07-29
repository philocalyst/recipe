mod __bundled_units {
    use super::*;
    pub fn get_bundled() -> UnitsFile {
        UnitsFile {
            default_system: Some(System::Metric),
            si: Some(SI {
                prefixes: Some(
                    EnumMap::from_array([
                        vec!["kilo".to_string()],
                        vec!["hecto".to_string()],
                        vec!["deca".to_string()],
                        vec!["deci".to_string()],
                        vec!["centi".to_string()],
                        vec!["milli".to_string()],
                    ]),
                ),
                symbol_prefixes: Some(
                    EnumMap::from_array([
                        vec!["k".to_string()],
                        vec!["h".to_string()],
                        vec!["da".to_string()],
                        vec!["d".to_string()],
                        vec!["c".to_string()],
                        vec!["m".to_string()],
                    ]),
                ),
                precedence: Precedence::default(),
            }),
            fractions: Some(Fractions {
                all: None,
                metric: Some(FractionsConfigWrapper::Toggle(false)),
                imperial: Some(FractionsConfigWrapper::Toggle(true)),
                quantity: {
                    let mut m = HashMap::with_capacity(2usize);
                    m.insert(
                        PhysicalQuantity::Temperature,
                        FractionsConfigWrapper::Toggle(false),
                    );
                    m.insert(
                        PhysicalQuantity::Time,
                        FractionsConfigWrapper::Toggle(false),
                    );
                    m
                },
                unit: {
                    let mut m = HashMap::with_capacity(3usize);
                    m.insert(
                        "lb".to_string(),
                        FractionsConfigWrapper::Custom(FractionsConfigHelper {
                            enabled: None,
                            accuracy: None,
                            max_denominator: Some(8u8),
                            max_whole: None,
                        }),
                    );
                    m.insert(
                        "tbsp".to_string(),
                        FractionsConfigWrapper::Custom(FractionsConfigHelper {
                            enabled: None,
                            accuracy: None,
                            max_denominator: Some(3u8),
                            max_whole: Some(4u32),
                        }),
                    );
                    m.insert(
                        "tsp".to_string(),
                        FractionsConfigWrapper::Custom(FractionsConfigHelper {
                            enabled: None,
                            accuracy: None,
                            max_denominator: Some(8u8),
                            max_whole: Some(5u32),
                        }),
                    );
                    m
                },
            }),
            extend: None,
            quantity: vec![
                QuantityGroup { quantity : PhysicalQuantity::Volume, best :
                Some(BestUnits::BySystem { metric : vec!["ml".to_string(), "l"
                .to_string()], imperial : vec!["cup".to_string(), "tsp".to_string(),
                "tbsp".to_string()], }), units : Some(Units::BySystem { metric :
                vec![UnitEntry { names : vec![Arc::from("liter"), Arc::from("liters"),
                Arc::from("litre"), Arc::from("litres")], symbols : vec![Arc::from("l"),
                Arc::from("L")], aliases : vec![], ratio : 1f64, difference : 0f64,
                expand_si : true, }], imperial : vec![UnitEntry { names :
                vec![Arc::from("teaspoon"), Arc::from("teaspoons")], symbols :
                vec![Arc::from("tsp"), Arc::from("tsp.")], aliases : vec![], ratio :
                0.004928921f64, difference : 0f64, expand_si : false, }, UnitEntry {
                names : vec![Arc::from("tablespoon"), Arc::from("tablespoons")], symbols
                : vec![Arc::from("tbsp"), Arc::from("tbsp."), Arc::from("tbs"),
                Arc::from("tbs.")], aliases : vec![], ratio : 0.014786764f64, difference
                : 0f64, expand_si : false, }, UnitEntry { names :
                vec![Arc::from("fluid ounce"), Arc::from("fluid ounces")], symbols :
                vec![Arc::from("fl oz"), Arc::from("fl. oz."), Arc::from("fl. oz"),
                Arc::from("fl oz.")], aliases : vec![], ratio : 0.029573529f64,
                difference : 0f64, expand_si : false, }, UnitEntry { names :
                vec![Arc::from("cup"), Arc::from("cups")], symbols :
                vec![Arc::from("c")], aliases : vec![], ratio : 0.236588236f64,
                difference : 0f64, expand_si : false, }, UnitEntry { names :
                vec![Arc::from("pint"), Arc::from("pints")], symbols :
                vec![Arc::from("pt")], aliases : vec![], ratio : 0.473176473f64,
                difference : 0f64, expand_si : false, }, UnitEntry { names :
                vec![Arc::from("quart"), Arc::from("quarts")], symbols :
                vec![Arc::from("qt")], aliases : vec![], ratio : 0.946352946f64,
                difference : 0f64, expand_si : false, }, UnitEntry { names :
                vec![Arc::from("gallon"), Arc::from("gallons")], symbols :
                vec![Arc::from("gal")], aliases : vec![], ratio : 3.785411784f64,
                difference : 0f64, expand_si : false, }], unspecified : vec![], }), },
                QuantityGroup { quantity : PhysicalQuantity::Length, best :
                Some(BestUnits::BySystem { metric : vec!["cm".to_string(), "mm"
                .to_string(), "m".to_string()], imperial : vec!["in".to_string(), "ft"
                .to_string()], }), units : Some(Units::BySystem { metric : vec![UnitEntry
                { names : vec![Arc::from("meter"), Arc::from("meters"),
                Arc::from("metre"), Arc::from("metres")], symbols : vec![Arc::from("m")],
                aliases : vec![], ratio : 1f64, difference : 0f64, expand_si : true, }],
                imperial : vec![UnitEntry { names : vec![Arc::from("foot"),
                Arc::from("feet")], symbols : vec![Arc::from("ft"), Arc::from("'")],
                aliases : vec![], ratio : 0.3048f64, difference : 0f64, expand_si :
                false, }, UnitEntry { names : vec![Arc::from("inch"),
                Arc::from("inches")], symbols : vec![Arc::from("in"), Arc::from("\"")],
                aliases : vec![], ratio : 0.0254f64, difference : 0f64, expand_si :
                false, }], unspecified : vec![], }), }, QuantityGroup { quantity :
                PhysicalQuantity::Mass, best : Some(BestUnits::BySystem { metric :
                vec!["mg".to_string(), "g".to_string(), "kg".to_string()], imperial :
                vec!["oz".to_string(), "lb".to_string()], }), units :
                Some(Units::BySystem { metric : vec![UnitEntry { names :
                vec![Arc::from("gram"), Arc::from("grams")], symbols :
                vec![Arc::from("g")], aliases : vec![], ratio : 1f64, difference : 0f64,
                expand_si : true, }], imperial : vec![UnitEntry { names :
                vec![Arc::from("ounce"), Arc::from("ounces")], symbols :
                vec![Arc::from("oz"), Arc::from("oz.")], aliases : vec![], ratio :
                28.349523125f64, difference : 0f64, expand_si : false, }, UnitEntry {
                names : vec![Arc::from("pound"), Arc::from("pounds")], symbols :
                vec![Arc::from("lb"), Arc::from("lb.")], aliases : vec![], ratio :
                453.59237f64, difference : 0f64, expand_si : false, }], unspecified :
                vec![], }), }, QuantityGroup { quantity : PhysicalQuantity::Time, best :
                Some(BestUnits::Unified(vec!["s".to_string(), "h".to_string(), "min"
                .to_string(), "d".to_string()])), units :
                Some(Units::Unified(vec![UnitEntry { names : vec![Arc::from("second"),
                Arc::from("seconds")], symbols : vec![Arc::from("s"), Arc::from("sec")],
                aliases : vec![Arc::from("secs")], ratio : 1f64, difference : 0f64,
                expand_si : false, }, UnitEntry { names : vec![Arc::from("minute"),
                Arc::from("minutes")], symbols : vec![Arc::from("min")], aliases :
                vec![Arc::from("mins")], ratio : 60f64, difference : 0f64, expand_si :
                false, }, UnitEntry { names : vec![Arc::from("hour"),
                Arc::from("hours")], symbols : vec![Arc::from("h")], aliases : vec![],
                ratio : 3600f64, difference : 0f64, expand_si : false, }, UnitEntry {
                names : vec![Arc::from("day"), Arc::from("days")], symbols :
                vec![Arc::from("d")], aliases : vec![], ratio : 86400f64, difference :
                0f64, expand_si : false, }])), }, QuantityGroup { quantity :
                PhysicalQuantity::Temperature, best : Some(BestUnits::BySystem { metric :
                vec!["C".to_string()], imperial : vec!["F".to_string()], }), units :
                Some(Units::BySystem { metric : vec![UnitEntry { names :
                vec![Arc::from("celsius")], symbols : vec![Arc::from("°C"),
                Arc::from("ºC"), Arc::from("℃"), Arc::from("C")], aliases : vec![],
                ratio : 1f64, difference : 273.15f64, expand_si : false, }], imperial :
                vec![UnitEntry { names : vec![Arc::from("fahrenheit")], symbols :
                vec![Arc::from("°F"), Arc::from("ºF"), Arc::from("℉"),
                Arc::from("F")], aliases : vec![], ratio : 0.55555555556f64, difference :
                459.67f64, expand_si : false, }], unspecified : vec![], }), }
            ],
        }
    }
}
