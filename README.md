# Guessing Game
*by Kalpa Behera*
---

```mermaid
---
title: Flowchart of Guessing Fruit Game
---
flowchart LR
    subgraph import[import libraries]
    direction TB
    std[Use IO module
    from std library]

    rand[Import all 
    random functions, methods
     from rand library]
    end
    std --> rand

    subgraph select[Select random Fruit from list]
    direction TB

    fruitslist[Make list
    of fruits]

    randNo[Generate
    random number]

    randomfruit[get fruit
    name from 
    list of fruits 
    based on
    random number
    ]
    end

    fruitslist --> randNo --> randomfruit
    
    inputfromuser{take input
    from user}



    errorHandling[Error Handling]

    check{Check 
    user input fruit
    with random fruit}

    Checkfruit{Check fruit 
    is available 
    or not in the list}

    Nofruitavailable[Fruit entered
     is not 
     in the list]

    Yesfruitavailable[print 
    you are a winner]

    Finish[Finish]

    tryagain{Do you want 
    to try again}



    import --> select --> inputfromuser --Input Error --> errorHandling 

    inputfromuser -->Checkfruit

    Checkfruit -- yes --> check --> Yesfruitavailable --Yes --> Finish

    check -- No --> tryagain
    Checkfruit -- No --> Nofruitavailable --> tryagain

    tryagain -- No --> Finish

    tryagain -- Yes --> inputfromuser


```

```mermaid
---
title: Flowchart of Error Handling
---
graph LR
    subgraph preprocess[Preprocess input Data]
    
    trim[Trim the 
    extra space of 
    input string]

    convert[Convert the 
    input string to 
    lower case]

    trim --> convert
    end

    checkinput{Check recieved 
    input is correct
    or not}

    printinput[Print the input]

    catcherror[Catch Error
    and print]

    preprocess --> checkinput

    checkinput -- Yes --> printinput
    checkinput -- No --> catcherror

```


> To take input from users.

`use std::io;`

*Use **io** modules from **std** library.*

> We want to generate Random number.

`use rand::prelude::*;`

*From **rand** library we will take **prelude** item.
From **prelude** item take all methods.*

> Let's take a list of fruits.

`let guess_list = ["grapes","banana","oranges"];`

> Create a random number.

`let mut rng = thread_rng();`

***thread_rng()** is a method to generate random number.*

> Create a random number in a range.

`let index = rng.gen_rang(0..guess_list.length());`

***gen_rang** will create random number between 0 and length of guess_list.*

*Here 0 is included but length of guess_list excluded.*

> Find random fruit

`let random_fruit = guess_list[index];`

*Random fruit will be selected from guess_list based upon random number saved in index*

> Take input from user, compare with random_fruit.

`let mut input = String::new()`

*Input type String*

> Check input type whether it's correct input or not(Error Handling).

```Rust
match io::stdin().read_line(&mut input){
    ok(_)=>{
        let fruit_selected = input.trim().to_lowercase();
        println!("Fruit Selected",fruit_selected)
    }
    Err(error)=>{
        println("Error is {}",error);
    }
}
```
***ok(_)** Here "_" used to take all input type.*

***input.trim()** is used to trim extra spaces.*

***input.to_lowercase()** is used to convert each letter to lower case.*

```Rust
    Err(error)=>{
        println!("Error is {}",error);
    }
```

***Err** will catch the error & **println!** will print the error*















