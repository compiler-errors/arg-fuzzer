
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3723(_: S2, _: S3, _: S6, _: S7) {}
        
        fn test3723() { foo3723(S4, S1, S6, S2, S8); }
    