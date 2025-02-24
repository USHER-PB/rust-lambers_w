# IMPLEMENTING THE LAMBERS W FUNCTION IN RUST

## INTRODUCTION

- The lambers w function also call as the omega function is a function that defines that e^w(x) * w(x) = x where e is the exponential .

### HOW IS IT RELATED TO MY CODE ??

- firt start by cloning the repo using the command

          git clone https://github.com/USHER-PB/rust-lambersw.git.
  when this is done continue the steps under

- At first we should know that we are trying to find your value of w not forgeting that it is initialise at the begining as the value of x .
  
- Here x is an argument that the user will input .These value of x is pass as an argument using the structopt . 

- then we will see if our value of x is matching with the normal limit that is -1/e < x < 0; So this means that your value should be less than 0 and less than -1/e  and if not we are using the enum Error from the file error.rs  that will help us to handle errors in our code 

- we can also observe that we are using the raphson method for iteration so as to be accurate in our output value. We are having the  MAX_ITER that dirrects us  the number of iteration our loop will take and tolerance that tells defines where our code is to suppose to be succesful

- we will initialise a new variable f = w*e(w) -x and f_prime to be its derivative so as to find our new value of w that is  w = w -f/f_prime

To run the code you can either place x before your value or input as define in the code that is 

           cargo run --x=-0.2  

         OR
           cargo run -- --input=-0.2
the = sign is due to the fact that you inputed value is negative 

- To run the Dockerfile first start by building it using

           docker build -t <name of your image>

- and finally run it

           docker run -it --name <name of your choice> <name of your image>
