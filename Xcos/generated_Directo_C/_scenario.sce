
// initial output values 
y1 = zeros(1, 1);


// initial input values 
t = 0.;
u1 = zeros(1, 1);

// preserve the API by discarding const and range analysis
//EMX?: emx_var_nopropagation(t, 'all');
//EMX?: emx_var_nopropagation(y1, 'all');
//EMX?: emx_var_nopropagation(u1, 'all');

// initialize all the blocks 


// generating inputs
[t, u1] = _inputs();

[y1] = (t, u1);

// print out values
_outputs(t, y1);

