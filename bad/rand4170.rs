
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4170(_: S2, _: S8) {}
        
        fn test4170() { foo4170(S4, S2, S0); }
    