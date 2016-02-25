initSidebarItems({"trait":[["FixedSizeArray","Utility trait implemented only on arrays of fixed sizeThis trait can be used to implement other traits on fixed-size arrays without causing much metadata bloat.The trait is marked unsafe in order to restrict implementors to fixed-size arrays. User of this trait can assume that implementors have the exact layout in memory of a fixed size array (for example, for unsafe initialization).Note that the traits AsRef and AsMut provide similar methods for types that may not be fixed-size arrays. Implementors should prefer those traits instead."]]});