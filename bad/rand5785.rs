
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5785(_: S2, _: S6) {}
        
        fn test5785() { foo5785(S5, S3, S4, S2, S5, S4); }
    