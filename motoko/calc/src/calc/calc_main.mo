// This single-cell calculator defines one calculator instruction per
// public entry point (add, sub, mul, div).

// Create a simple Calc actor.
actor Calc {
  var cell : Int = 0;

  // Define functions to add, subtract, multiply, and divide
  public func add(n:Int) : async Int { cell += n; cell };
  public func sub(n:Int) : async Int { cell -= n; cell };
  public func mul(n:Int) : async Int { cell *= n; cell };
  public func div(n:Int) : async ?Int {
    if ( n == 0 ) {
      // null encodes div-by-zero error
      return null
    } else {
      cell /= n; ?cell
    }
  };

    // Clear the calculator and reset to zero
    public func clearall() : async Int {
    if (cell : Int != 0)
       cell -= cell;
    return cell};
 };

