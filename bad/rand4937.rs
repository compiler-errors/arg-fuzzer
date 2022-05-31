
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4937(_: S1, _: S2, _: S7, _: S8) {}
        
        fn test4937() { foo4937(S0, S0, S5, S7, S6); }
    