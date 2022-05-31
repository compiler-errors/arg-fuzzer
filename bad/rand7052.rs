
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7052(_: S3, _: S6, _: S7) {}
        
        fn test7052() { foo7052(S1, S2, S3, S4, S7, S8); }
    