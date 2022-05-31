
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9797(_: S1, _: S4, _: S5, _: S8) {}
        
        fn test9797() { foo9797(S1, S1, S4, S0, S1); }
    