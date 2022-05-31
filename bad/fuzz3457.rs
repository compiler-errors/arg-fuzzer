
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3457(_: S8, _: S4, _: S2, _: S7, _: S1) {}
        
        fn test3457() { foo3457(S2, S6, S3, S6, S7, S8, S3); }
    