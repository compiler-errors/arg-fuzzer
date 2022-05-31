
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8390(_: S4, _: S3, _: S6) {}
        
        fn test8390() { foo8390(S4, S5, S2, S3, S1, S7, S8); }
    