
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2999(_: S1, _: S4, _: S5, _: S7) {}
        
        fn test2999() { foo2999(S6, S5, S2, S4, S0, S2); }
    