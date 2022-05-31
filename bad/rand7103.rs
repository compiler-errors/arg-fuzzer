
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7103(_: S5, _: S3, _: S2) {}
        
        fn test7103() { foo7103(S7, S1, S3, S7, S0); }
    