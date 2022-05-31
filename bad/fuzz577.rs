
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo577(_: S4, _: S5, _: S7, _: S8) {}
        
        fn test577() { foo577(S7, S5, S2, S1, S4, S3); }
    