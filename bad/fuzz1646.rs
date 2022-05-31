
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1646(_: S4, _: S2) {}
        
        fn test1646() { foo1646(S1, S4, S5, S6); }
    