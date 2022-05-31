
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo646(_: S2, _: S6, _: S7) {}
        
        fn test646() { foo646(S5, S1, S4, S8, S4, S2, S8); }
    