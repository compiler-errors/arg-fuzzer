
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6337(_: S3, _: S2, _: S7) {}
        
        fn test6337() { foo6337(S5, S1, S5, S3, S6, S3); }
    