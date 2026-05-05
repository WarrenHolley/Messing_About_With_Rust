//Tower Of Hanoi - Just messing about.
//--------------------------------------------------
// Couldn't find my old copy, so threw this together.
// Needs -serious- upgrades to be 'good'.
// Todo: Argparse, sanity checking, find a way to print mid-TOH recursion to monitor steps. (Global vars, like by Arb-TOH?)
// May upgrade to the Arbitrary-TOH, just to experiment with porting that self-nerd-snipe.

fn init_towers( num_rings:u8 ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
  // Just returns three vectors, each corresponding to a tower.
  // First has all rings, in descending order, (Starting at size=num_rings, doesn't have a zero)
  // Second two are empty.
  // Eg: 5 -> [5,4,3,2,1], [], []
  
  let mut tow1: Vec<u8> = Vec::new();
  let     tow2: Vec<u8> = Vec::new();
  let     tow3: Vec<u8> = Vec::new();
  
  for i in (1..num_rings+1).rev() { // Can't find a [::-1] operator?
    tow1.push(i);
  }

  return (tow1, tow2, tow3)
}

// (I somewhat disagree with the reasoning behind the compiler's warnings of non-snake-case. Suppressing just out of spite.)
#[allow(non_snake_case)]
fn TOH( src: &mut Vec<u8>, dst: &mut Vec<u8>, aux: &mut Vec<u8>, count:u8 ) -> u64 {
  // First move all rings to aux, minus the last,
  //  Then move the final ring to dst, 
  //  Then move all rings in aux to dst
  // Count = Number of rings to move.
  // src/dst/aux are the peg vectors.
  
  let mut counter:u64 = 0;
  
  if count > 1 { // Move all but the last into the aux peg. (swap dst/aux)
    counter += TOH( src, aux, dst, count - 1 )
  }
  
  // Then move the largest ring in src into dst.
  let ring = src.pop().unwrap(); // Unsafe if src empty? WGH:Upgrade to Expect?
  dst.push( ring );
  counter += 1;
  
  if count > 1 { // Move everything in the aux peg onto dst.
    counter += TOH( aux, dst, src, count - 1 )
  }
  
  return counter;
  
  // Can't diagnose which tower is T1/2/3 given just the src/dst.
  // Experimenting with giving a second reference, immutable or mutable, in the function to track it just caused the compile to copmlain.
  // Easiest method may be to give the vectors as a tuple? Give an index list to dereference src/dst/aux.
  // WGH:Look into this later.
}
  
//--------------------------------------------------

fn main() {
  // Just fucking about here, so set n=5. Set up argparse later.
  let num_rings = 30;

  // Setup inital vectors
  let (mut tow1, mut tow2, mut tow3) = init_towers( num_rings );

  //Print initial state
  println!("T1: {:?}", tow1);
  println!("T2: {:?}", tow2);
  println!("T3: {:?}", tow3);
  
  // And count total number of moves. Should be (2^n)-1, or f(5)=31.
  let counter = TOH( &mut tow1, &mut tow3, &mut tow2, num_rings );
  
  //Print final state
  println!();
  println!("T1: {:?}", tow1);
  println!("T2: {:?}", tow2);
  println!("T3: {:?}", tow3);
  println!();
  println!("Total Moves: {:?}", counter);
}

