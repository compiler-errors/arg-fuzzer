
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5338(_: S1, _: S5, _: S0) {}
        
        fn test5338() { foo5338(S6, S8, S5, S7, S1, S4, S2); }
    