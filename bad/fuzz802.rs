
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo802(_: S4, _: S6, _: S5) {}
        
        fn test802() { foo802(S1, S7, S3, S4, S8, S5, S2); }
    