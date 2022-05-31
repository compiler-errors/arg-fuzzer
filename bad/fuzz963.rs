
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo963(_: S3, _: S6) {}
        
        fn test963() { foo963(S1, S3, S5, S8); }
    