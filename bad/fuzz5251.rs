
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5251(_: S8, _: S3, _: S7, _: S5) {}
        
        fn test5251() { foo5251(S3, S2, S4, S1, S3, S2, S7); }
    