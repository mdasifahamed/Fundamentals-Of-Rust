fn main() {
    // Slicing in rust 
    /*
        A dunction to rteurn first word if the thereis  only one word return the word
        it will look for space after the first word and end the serching after first space 
        otherif here is no space the whole word is the forst word
    */

    // this function is going to return the lenght of the firts word word 
    fn first_word(word: &str) -> &str {
        let bytes = word.as_bytes(); // converting the String to the bytes array as we wil look for the element by element
        
        for(i,&items) in bytes.iter().enumerate(){
            if items == b' '{
                /* Here is slice Work for we stoped at the 
                    first space and get the det index of the first space
                    before the first space all the indexs and elemnet is related to the 
                    first word so from the first to the space index slaincing we will get first word
                    slicing work like [first index /elemnet to last given index/elment -1]
                    if we have i = 6 and strat from the 0 then it will be 0 to 6-1 = 5 or 0 to 5
                */
                return &word[0..i];
            }
           
        }
        &word[..]
    }


    let  sentence = "Rust is Complex"; // This will be converted to array of bytes having elemnets of 14 and first space is at 5 so 0 to 5-1 =4 in the function it will 0 to 4 
    let word = first_word(&sentence);
    println!("The First word Is '{:}'", word);

    fn last_word(sen: &str) -> &str {

        let  bytes = sen.as_bytes();

        for(i,&items) in bytes.iter().rev().enumerate(){
            if items == b' '{
                return &sen[i+1..]; // we have update the index as it stops at the index of first space the the we and we are starting it  so the next index is the first element of the last word .
            }
        }
          

        &sen[..]
    }
    
    let lastWord = last_word(&sentence);
    println!("Last Word Is '{:}'", lastWord);


}
