
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo35(_: S1, _: S3, _: S4, _: S5, _: S7) {}
        
        fn test35() { foo35(S2, S3, S1, S5, S1, S5, S4); }
    