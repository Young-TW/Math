let a = 1;
function fact(n){
    if (n === 0){
        return a;
    }else{
        a = n * a;
        n--;
        return fact(n);
    }
}

console.log(fact(5));