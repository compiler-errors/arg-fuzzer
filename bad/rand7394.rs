
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7394(_: S7, _: S3) {}
        
        fn test7394() { foo7394(S1, S4, S6, S7, S8); }
    