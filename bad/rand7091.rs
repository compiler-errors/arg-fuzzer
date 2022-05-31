
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7091(_: S3, _: S7, _: S8) {}
        
        fn test7091() { foo7091(S1, S2, S3, S4); }
    