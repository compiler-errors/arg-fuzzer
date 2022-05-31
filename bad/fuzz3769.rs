
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3769(_: S1, _: S2, _: S3, _: S4, _: S6, _: S7, _: S8) {}
        
        fn test3769() { foo3769(S3, S6, S3, S3, S5, S2, S4, S2); }
    