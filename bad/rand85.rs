
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo85(_: S2, _: S6, _: S3, _: S8) {}
        
        fn test85() { foo85(S5, S7, S8, S6, S3, S7, S1, S1); }
    