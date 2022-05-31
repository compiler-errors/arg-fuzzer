
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7257(_: S1, _: S3, _: S5) {}
        
        fn test7257() { foo7257(S5, S1, S0, S0); }
    