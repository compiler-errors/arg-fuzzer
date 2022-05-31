
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15753(_: S1, _: S2, _: S5, _: S6) {}
        
        fn test15753() { foo15753(S1, S3, S1, S2, S0, S1, S2); }
    