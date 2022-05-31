
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7624(_: S3, _: S1, _: S1, _: S2, _: S3) {}
        
        fn test7624() { foo7624(S6, S0, S6, S2, S4, S1, S2); }
    