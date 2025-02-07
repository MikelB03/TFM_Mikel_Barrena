function [t, u1] = _inputs()
    //EMX?: emx_func_file('_inputs.c')
    
    t = rand();
    u1 = (rand(1, 1));
endfunction

function _outputs(t, y1)
    //EMX?: emx_func_file('_outputs.c')
    
    disp(t);
    disp(y1);
endfunction

function [y1] = _frenoEmergencia(t, u1)
    //EMX?: emx_func_file('_frenoEmergencia.c')
    
    
    // update outputs 

    // 5430236c:193530b8804:-7fec
    // scs_m(list("objs",1,"model","rpar","objs",2))
    // ValorPotencio1/
    // cstblk4_m
    tmp2 = 10;

    // 5430236c:193530b8804:-7fea
    // scs_m(list("objs",1,"model","rpar","objs",3))
    // ValorPotencio1/
    // cstblk4_m
    tmp3 = 20;

    // 5430236c:193530b8804:-7fe4
    // scs_m(list("objs",1,"model","rpar","objs",4))
    // ValorPotencio1/
    // relational_op
    tmp4 = double(u1 > tmp2);

    // 5430236c:193530b8804:-7fe0
    // scs_m(list("objs",1,"model","rpar","objs",5))
    // ValorPotencio1/
    // relational_op
    tmp5 = double(u1 < tmp3);

    // 5430236c:193530b8804:-7fdc
    // scs_m(list("objs",1,"model","rpar","objs",6))
    // ValorPotencio1/
    // logicalop
    tmp6 = double(tmp4 & tmp5);

    // 5430236c:193530b8804:-7fd8
    // scs_m(list("objs",1,"model","rpar","objs",19))
    // ValorPotencio1/
    // xcg_output_sim
    y1 = tmp6;
endfunction

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

[y1] = _frenoEmergencia(t, u1);

// print out values
_outputs(t, y1);

