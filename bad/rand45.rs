
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo45(_: S3, _: S4) {}
        
        fn test45() { foo45(S2, S3, S5, S6, S7); }
    