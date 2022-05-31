
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8284(_: S2, _: S3, _: S8) {}
        
        fn test8284() { foo8284(S7, S3, S5, S2, S1, S1, S4); }
    