
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15185(_: S1, _: S3, _: S3) {}
        
        fn test15185() { foo15185(S5, S6, S1, S7, S2, S4, S8); }
    