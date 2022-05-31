
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9512(_: S1, _: S2, _: S3, _: S5, _: S7) {}
        
        fn test9512() { foo9512(S6, S1, S0, S6, S5, S6); }
    