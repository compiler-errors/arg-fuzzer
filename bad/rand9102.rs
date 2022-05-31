
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9102(_: S1, _: S5, _: S7, _: S8) {}
        
        fn test9102() { foo9102(S2, S2, S2, S0, S1, S0); }
    