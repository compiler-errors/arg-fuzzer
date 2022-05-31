
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7321(_: S3, _: S1, _: S3, _: S3) {}
        
        fn test7321() { foo7321(S1, S2, S7, S5, S2, S3, S5); }
    