
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4742(_: S7, _: S5, _: S2) {}
        
        fn test4742() { foo4742(S6, S4, S5, S7, S3, S3, S3); }
    