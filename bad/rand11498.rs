
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11498(_: S1, _: S3, _: S4) {}
        
        fn test11498() { foo11498(S6, S8, S1, S7, S3, S2); }
    