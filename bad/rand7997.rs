
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7997(_: S7, _: S0, _: S1, _: S7, _: S2, _: S4) {}
        
        fn test7997() { foo7997(S1, S2, S6, S2, S1, S7); }
    