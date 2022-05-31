
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5042(_: S5, _: S8, _: S1) {}
        
        fn test5042() { foo5042(S7, S5, S0, S1); }
    