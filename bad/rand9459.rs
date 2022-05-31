
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9459(_: S5, _: S4, _: S0, _: S5) {}
        
        fn test9459() { foo9459(S4, S7, S6, S5, S4, S1, S5); }
    