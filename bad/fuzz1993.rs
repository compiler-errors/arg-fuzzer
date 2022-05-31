
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1993(_: S6, _: S3) {}
        
        fn test1993() { foo1993(S1, S2, S6, S8); }
    