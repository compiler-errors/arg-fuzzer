
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo369(_: S6, _: S4) {}
        
        fn test369() { foo369(S2, S5, S6, S7, S3, S8, S1); }
    