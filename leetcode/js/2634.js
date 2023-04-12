/*
    2634. Filter Elements from Array
    --------------------------------
*/

var filter = function(arr, fn) {
    const a = new Array();
    for (let i = 0; i < arr.length; i++) {
        if (fn.length == 2) {
            if (fn(arr[i], i)) {
                a.push(arr[i])
            }
        } else {
            if (fn(arr[i])) {
                a.push(arr[i])
            }
        }
    }
    return a
};
