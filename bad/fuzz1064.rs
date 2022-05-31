
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1064(_: S4, _: S7) {}
        
        fn test1064() { foo1064(S6, S3, S5, S6, S8, S4, S5, S2); }
    