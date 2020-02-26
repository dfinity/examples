actor {
    public func location(cities : [Text]) : async Text {
        return "Hello, from " # (debug_show cities) # "!";
    };
    public func location_pretty(cities : [Text]) : async Text {
        var str = "Hello from ";
        for (city in cities.vals()) {
                str := str # city #", ";
        };
        return str # "bon voyage!";
    }
};

