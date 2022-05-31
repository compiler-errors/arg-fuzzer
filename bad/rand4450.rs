
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4450(_: S1, _: S3, _: S7) {}
        
        fn test4450() { foo4450(S5, S4, S3, S2, S8, S6); }
    